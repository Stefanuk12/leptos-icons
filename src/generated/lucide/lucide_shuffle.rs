use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 18h1.4c1.3 0 2.5-.6 3.3-1.7l6.1-8.6c.7-1.1 2-1.7 3.3-1.7H22" ></ path > < path d = "m18 2 4 4-4 4" ></ path > < path d = "M2 6h1.9c1.5 0 2.9.9 3.6 2.2" ></ path > < path d = "M22 18h-5.9c-1.3 0-2.6-.7-3.3-1.8l-.5-.8" ></ path > < path d = "m18 14 4 4-4 4" ></ path > < / > } } pub const LucideShuffle : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2")] } ;