use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 3v11" ></ path > < path d = "M10 9H7a1 1 0 0 1 0-6h8" ></ path > < path d = "M14 3v11" ></ path > < path d = "m18 14 4 4H2" ></ path > < path d = "m22 18-4 4" ></ path > < / > } } pub const LUCIDE_PILCROW_RIGHT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;