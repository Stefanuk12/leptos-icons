use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" y = "3" width = "18" x = "3" ></ rect > < path d = "M3 9h18" ></ path > < / > } } pub const LUCIDE_PANEL_TOP : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;