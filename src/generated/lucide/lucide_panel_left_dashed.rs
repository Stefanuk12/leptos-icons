use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "18" width = "18" x = "3" rx = "2" ></ rect > < path d = "M9 14v1" ></ path > < path d = "M9 19v2" ></ path > < path d = "M9 3v2" ></ path > < path d = "M9 9v1" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT_DASHED : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;