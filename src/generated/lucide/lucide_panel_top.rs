use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" width = "18" y = "3" x = "3" ></ rect > < path d = "M3 9h18" ></ path > < / > } } pub const LUCIDE_PANEL_TOP : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;