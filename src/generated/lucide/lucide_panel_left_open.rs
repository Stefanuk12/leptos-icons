use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" height = "18" width = "18" x = "3" ></ rect > < path d = "M9 3v18" ></ path > < path d = "m14 9 3 3-3 3" ></ path > < / > } } pub const LUCIDE_PANEL_LEFT_OPEN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;