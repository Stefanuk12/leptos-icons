use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" rx = "2" x = "3" y = "3" ></ rect > < path d = "M15 14v1" ></ path > < path d = "M15 19v2" ></ path > < path d = "M15 3v2" ></ path > < path d = "M15 9v1" ></ path > < / > } } pub const LUCIDE_PANEL_RIGHT_DASHED : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;