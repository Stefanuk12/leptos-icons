use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 6 4 14" ></ path > < path d = "M12 6v14" ></ path > < path d = "M8 8v12" ></ path > < path d = "M4 4v16" ></ path > < / > } } pub const LUCIDE_LIBRARY : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;