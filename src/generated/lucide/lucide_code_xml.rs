use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m18 16 4-4-4-4" ></ path > < path d = "m6 8-4 4 4 4" ></ path > < path d = "m14.5 4-5 16" ></ path > < / > } } pub const LUCIDE_CODE_XML : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;