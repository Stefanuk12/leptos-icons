use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" rx = "2" height = "18" width = "18" ></ rect > < path d = "M9 14v1" ></ path > < path d = "M9 19v2" ></ path > < path d = "M9 3v2" ></ path > < path d = "M9 9v1" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT_DASHED : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;