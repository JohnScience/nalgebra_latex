[![crates.io](https://img.shields.io/crates/v/nalgebra_latex.svg)][`nalgebra_latex`]
[![crates.io](https://img.shields.io/crates/d/nalgebra_latex.svg)][`nalgebra_latex`]

# [`nalgebra`] extension for LaTeX

The crate provides several robust formatters for [`nalgebra::Matrix`] as well as
several LaTeX environments for customization.

On top of that, the crate offers feature-gated support for [`nalgebra_linsys`] and [`evcxr`].

## Example

```rust
	use nalgebra::matrix;
	use nalgebra_latex::fmt::{PlainMatrixFormatter, LatexFormatter};

	let mut s = String::new();
	let m = matrix!(
		1,2,3,4;
		5,6,7,8;
		9,10,11,12;
	);
	PlainMatrixFormatter::write_latex(&mut s, &m).unwrap();
	assert_eq!(s, r"\begin{matrix}$1$&$2$&$3$&$4$\\$5$&$6$&$7$&$8$\\$9$&$10$&$11$&$12$\end{matrix}");
```

## What is [`nalgebra`]?

[`nalgebra`] is a general-purpose linear algebra library with transformations and statically-sized or dynamically-sized matrices.

## What is LaTeX?

LaTeX is a language for typesetting documents, especially scientific papers, and a document preparation system.

## Example of .tex code

```tex
% ...
\subsection*{H}
	\glossaryentry{hadamard_product}{Hadamard product}
	\begin{adjustwidth}{1em}{}
		\textbf{Field of study}: \textit{Mathematics. Linear Algebra. Matrix theory.} \\
		\textbf{Distinct meanings in other fields of study}: \textit{unspecified.} \\
		\textbf{Definitions}:
		\begin{adjustwidth}{1em}{} \leavevmode
			\begin{framed}
				For two \hyperlink{matrix}{\textit{matrices}} $A$ and $B$ of the same \hyperlink{dimension_of_matrix}{\textit{dimension}} $m \times n$, the \beingdefined{Hadamard product} $A \circ B$ (or $A \odot B$) is a \hyperlink{matrix}{\textit{matrix}} of the same \hyperlink{dimension_of_matrix}{\textit{dimension}} as the operands, with elements given by
				\begin{equation*}
					(A \circ B)_{ij} = (A \odot B)_{ij} = (A)_{ij}(B)_{ij}.
				\end{equation*}
				
				Source: \cite{wiki_hadamard_product_matrices}.
			\end{framed}
			\begin{framed}
				Let $A$ and $B$ be $m \times n$ \hyperlink{matrix}{\textit{matrices}} with entries in $C$. The \beingdefined{Hadamard product} is defined by $[A \circ B]_{ij}=[A]_{ij}[B]_{ij}$ for all $1 \leq i \leq m$, $1 \leq j \leq n$. \\ \vspace{1em}
				
				Source: \cite{emillion}.
			\end{framed}
		\end{adjustwidth}
	\end{adjustwidth} \vspace{1em}
% ...
```

### Output

![tex output](https://i.imgur.com/xptzo3h.jpg)

## Resources on LaTeX

* [LaTeX documentation on Overleaf](https://www.overleaf.com/learn)
* [`amsmath` matrix environments and inline matrices](https://www.overleaf.com/learn/latex/Matrices)

## Other useful links

* [The list of features on docs.rs](https://docs.rs/crate/nalgebra_latex/latest/features)

[`nalgebra_latex`]: https://crates.io/crates/nalgebra_latex
[`nalgebra`]: https://crates.io/crates/nalgebra
[what is latex]: https://www.overleaf.com/learn/latex/Learn_LaTeX_in_30_minutes#What_is_LaTeX.3F
[`nalgebra::Matrix`]: https://docs.rs/nalgebra/latest/nalgebra/base/struct.Matrix.html
[`nalgebra_linsys`]: https://crates.io/crates/nalgebra_linsys
[`evcxr`]: https://github.com/google/evcxr

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
