use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" height = "18" x = "3" y = "3" rx = "2" ></ rect > < path d = "M14 15h1" ></ path > < path d = "M19 15h2" ></ path > < path d = "M3 15h2" ></ path > < path d = "M9 15h1" ></ path > < / > } } pub const LUCIDE_PANEL_BOTTOM_DASHED : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;