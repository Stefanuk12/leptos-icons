use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 3v11" ></ path > < path d = "M10 9H7a1 1 0 0 1 0-6h8" ></ path > < path d = "M14 3v11" ></ path > < path d = "m18 14 4 4H2" ></ path > < path d = "m22 18-4 4" ></ path > < / > } } pub const LUCIDE_PILCROW_RIGHT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;