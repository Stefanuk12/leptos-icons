use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "4" width = "20" x = "2" height = "16" rx = "2" ></ rect > < path d = "M6 8h.01" ></ path > < path d = "M10 8h.01" ></ path > < path d = "M14 8h.01" ></ path > < / > } } pub const LUCIDE_APP_WINDOW_MAC : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;