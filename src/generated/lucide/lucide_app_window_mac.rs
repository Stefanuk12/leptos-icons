use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" width = "20" height = "16" y = "4" rx = "2" ></ rect > < path d = "M6 8h.01" ></ path > < path d = "M10 8h.01" ></ path > < path d = "M14 8h.01" ></ path > < / > } } pub const LUCIDE_APP_WINDOW_MAC : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24")] } ;