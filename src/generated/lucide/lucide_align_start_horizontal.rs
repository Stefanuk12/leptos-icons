use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "6" x = "4" rx = "2" height = "16" width = "6" ></ rect > < rect height = "9" y = "6" rx = "2" x = "14" width = "6" ></ rect > < path d = "M22 2H2" ></ path > < / > } } pub const LUCIDE_ALIGN_START_HORIZONTAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;