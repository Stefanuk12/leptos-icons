use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" y = "4" x = "2" width = "20" rx = "2" ></ rect > < path d = "M6 8h.01" ></ path > < path d = "M10 8h.01" ></ path > < path d = "M14 8h.01" ></ path > < / > } } pub const LUCIDE_APP_WINDOW_MAC : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24")] } ;