use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" rx = "2" x = "3" y = "3" width = "18" ></ rect > < path d = "M9 3v18" ></ path > < path d = "m14 9 3 3-3 3" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT_OPEN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24")] } ;