use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "6" width = "9" rx = "2" y = "14" height = "6" ></ rect > < rect x = "6" height = "6" width = "16" rx = "2" y = "4" ></ rect > < path d = "M2 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_START_VERTICAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24")] } ;