use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "5" rx = "2" width = "6" height = "14" x = "6" ></ rect > < rect height = "10" width = "6" y = "7" x = "16" rx = "2" ></ rect > < path d = "M2 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_JUSTIFY_START : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;