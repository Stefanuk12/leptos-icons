use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "18" width = "18" x = "3" rx = "2" ></ rect > < path d = "M3 15h18" ></ path > < / > } } pub const LUCIDE_PANEL_BOTTOM : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;