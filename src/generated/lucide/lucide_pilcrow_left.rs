use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 3v11" ></ path > < path d = "M14 9h-3a3 3 0 0 1 0-6h9" ></ path > < path d = "M18 3v11" ></ path > < path d = "M22 18H2l4-4" ></ path > < path d = "m6 22-4-4" ></ path > < / > } } pub const LUCIDE_PILCROW_LEFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;