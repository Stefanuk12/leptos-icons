use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "6" y = "6" x = "4" height = "16" ></ rect > < rect y = "6" width = "6" x = "14" height = "9" rx = "2" ></ rect > < path d = "M22 2H2" ></ path > < / > } } pub const LUCIDE_ALIGN_START_HORIZONTAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24")] } ;