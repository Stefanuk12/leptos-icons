use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "2" y = "3" height = "18" width = "18" ></ rect > < path d = "M14 15h1" ></ path > < path d = "M19 15h2" ></ path > < path d = "M3 15h2" ></ path > < path d = "M9 15h1" ></ path > < / > } } pub const LUCIDE_PANEL_BOTTOM_DASHED : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;