use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "14" rx = "2" width = "9" height = "6" x = "6" ></ rect > < rect height = "6" x = "6" width = "16" y = "4" rx = "2" ></ rect > < path d = "M2 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_START_VERTICAL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round")] } ;