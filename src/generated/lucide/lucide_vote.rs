use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m9 12 2 2 4-4" ></ path > < path d = "M5 7c0-1.1.9-2 2-2h10a2 2 0 0 1 2 2v12H5V7Z" ></ path > < path d = "M22 19H2" ></ path > < / > } } pub const LUCIDE_VOTE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;