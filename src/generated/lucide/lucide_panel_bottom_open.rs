use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" y = "3" height = "18" x = "3" ></ rect > < path d = "M3 15h18" ></ path > < path d = "m9 10 3-3 3 3" ></ path > < / > } } pub const LUCIDE_PANEL_BOTTOM_OPEN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;