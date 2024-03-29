[![crates.io](https://img.shields.io/crates/v/nalgebra_latex.svg)][`nalgebra_latex`]
[![crates.io](https://img.shields.io/crates/d/nalgebra_latex.svg)][`nalgebra_latex`]

# [`nalgebra`] extension for LaTeX

The crate provides several robust formatters for [`nalgebra::Matrix`] as well as
several LaTeX environments for customization.

On top of that, the crate offers feature-gated support for [`nalgebra_linsys`] and [`evcxr`].

## Example

```rust
use nalgebra_latex::{
	latex_writer::{Writer, LatexWriter, UnsafeWrite},
	latex_flavors::AmsLatex,
	latex_features::NoFeatures,
	latex_modes::{InnerParagraphMode, DisplayMathMode},
	latex_format,
};

let mut s = Writer::<AmsLatex,NoFeatures,InnerParagraphMode,String>::default();
latex_format!(
	#[on_format_error(unwrap)]
	s += "$$" ;
	|mut s: Writer::<AmsLatex,NoFeatures,DisplayMathMode,String>| {
		unsafe { s.write_str(r"\overrightarrow{x}" ) }?; 
		Ok(s)
	};
	// At the moment of writing string literals are being pushed inside of writers without validation o contents
	// However, it can change in the future
	" = (x_1,x_2)";
	"$$" ;
);
let (s,_no_features) = s.into_raw_parts();
assert_eq!(s, r"$$\overrightarrow{x} = (x_1,x_2)$$");
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

## Warning

The library is overhauled. It changes frequently and the documentation is not always available. Doc tests
(examples in the documentation) generally show working code.

A lot of previous work was put into trash to make it work better. The last commit before the breaking changes is [here](https://github.com/JohnScience/nalgebra_latex/tree/7f8a1c44619c7c9a82ab869e407352e432a24565).

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
