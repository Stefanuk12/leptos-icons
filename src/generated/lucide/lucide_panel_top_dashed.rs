use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" height = "18" width = "18" x = "3" ></ rect > < path d = "M14 9h1" ></ path > < path d = "M19 9h2" ></ path > < path d = "M3 9h2" ></ path > < path d = "M9 9h1" ></ path > < / > } } pub const LUCIDE_PANEL_TOP_DASHED : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;