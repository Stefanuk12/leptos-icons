use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "16" x = "2" width = "20" y = "4" ></ rect > < path d = "M6 8h.01" ></ path > < path d = "M10 8h.01" ></ path > < path d = "M14 8h.01" ></ path > < / > } } pub const LUCIDE_APP_WINDOW_MAC : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;