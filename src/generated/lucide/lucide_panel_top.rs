use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "3" y = "3" height = "18" width = "18" ></ rect > < path d = "M3 9h18" ></ path > < / > } } pub const LUCIDE_PANEL_TOP : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;