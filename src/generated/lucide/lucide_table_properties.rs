use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 3v18" ></ path > < rect rx = "2" y = "3" x = "3" width = "18" height = "18" ></ rect > < path d = "M21 9H3" ></ path > < path d = "M21 15H3" ></ path > < / > } } pub const LUCIDE_TABLE_PROPERTIES : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;