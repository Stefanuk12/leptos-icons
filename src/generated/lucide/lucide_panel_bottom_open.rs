use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" width = "18" height = "18" x = "3" ></ rect > < path d = "M3 15h18" ></ path > < path d = "m9 10 3-3 3 3" ></ path > < / > } } pub const LUCIDE_PANEL_BOTTOM_OPEN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;