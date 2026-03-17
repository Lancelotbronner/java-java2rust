/*
 * Copyright (C) 2015-2016 Federico Tomassetti
 * Copyright (C) 2017-2024 The JavaParser Team.
 *
 * This file is part of JavaParser.
 *
 * JavaParser can be used either under the terms of
 * a) the GNU Lesser General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 * b) the terms of the Apache License
 *
 * You should have received a copy of both licenses in LICENCE.LGPL and
 * LICENCE.APACHE. Please refer to those files for details.
 *
 * JavaParser is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 */

package javaparser;

import com.github.javaparser.ast.body.TypeDeclaration;
import com.github.javaparser.resolution.TypeSolver;
import com.github.javaparser.resolution.cache.Cache;
import com.github.javaparser.resolution.declarations.ResolvedReferenceTypeDeclaration;
import com.github.javaparser.resolution.model.SymbolReference;
import com.github.javaparser.symbolsolver.cache.GuavaCache;
import com.github.javaparser.symbolsolver.javaparsermodel.JavaParserFacade;
import com.github.javaparser.utils.SourceZip;
import com.google.common.cache.CacheBuilder;
import org.apache.commons.lang3.StringUtils;

import java.io.IOException;
import java.nio.file.Path;
import java.util.*;

/**
 *
 *
 * @author Christophe Bronner
 */
public class SourceZipTypeSolver implements TypeSolver {
	private static final int CACHE_SIZE_UNSET = -1;
	public final SourceZip sources;
	public final List<Path> paths = new ArrayList<>();
	public final HashMap<String, TypeDeclaration<?>> types = new HashMap<>();
	private String commonPrefix = "";
	private TypeSolver parent;

	public SourceZipTypeSolver(SourceZip sources) throws IOException {
		this(sources, CACHE_SIZE_UNSET);
	}

	public SourceZipTypeSolver(SourceZip sources, int cacheSizeLimit) throws IOException {
		this.sources = sources;
	}

	@Override
	public String toString() {
		return "SourceZipTypeSolver{" + "zipPath=" + sources.getZipPath() + ", parent=" + parent + '}';
	}

	@Override
	public TypeSolver getParent() {
		return parent;
	}

	@Override
	public void setParent(TypeSolver parent) {
		Objects.requireNonNull(parent);
		if (this.parent != null) {
			throw new IllegalStateException("This TypeSolver already has a parent.");
		}
		if (parent == this) {
			throw new IllegalStateException("The parent of this TypeSolver cannot be itself.");
		}
		this.parent = parent;
	}

	@Override
	public SymbolReference<ResolvedReferenceTypeDeclaration> tryToSolveType(String name) {
		parseIfNecessary();
		if (!name.startsWith(commonPrefix))
			return SymbolReference.unsolved();

		TypeDeclaration<?> td = types.get(name);
		if (td == null)
			return SymbolReference.unsolved();

		return SymbolReference.solved(JavaParserFacade.get(this).getTypeDeclaration(td));
	}

	@Override
	public SymbolReference<ResolvedReferenceTypeDeclaration> tryToSolveTypeInModule(
		String qualifiedModuleName,
		String simpleTypeName
	) {
		return tryToSolveType(qualifiedModuleName + "." + simpleTypeName);
	}

	public void parseIfNecessary() {
		if (!types.isEmpty())
			return;
		try {
			sources.parse((path, result) -> {
				paths.add(path);
				if (result.getResult().isEmpty())
					return;
				for (TypeDeclaration<?> td : result.getResult().get().getTypes())
					types.put(td.getFullyQualifiedName().orElse(td.getNameAsString()), td);
			});
			commonPrefix = StringUtils.getCommonPrefix(types.keySet().toArray(new String[0]));
		} catch (IOException e) {
			throw new RuntimeException(
				"Issue while parsing while type solving: " + sources.getZipPath().toAbsolutePath(),
				e);
		}
	}
}
