use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" rx = "2" y = "3" x = "3" ></ rect > < path d = "M14 9h1" ></ path > < path d = "M19 9h2" ></ path > < path d = "M3 9h2" ></ path > < path d = "M9 9h1" ></ path > < / > } } pub const LUCIDE_PANEL_TOP_DASHED : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24")] } ;