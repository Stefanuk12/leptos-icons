use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "6" y = "16" x = "5" width = "14" rx = "2" ></ rect > < rect y = "2" rx = "2" width = "10" height = "6" x = "7" ></ rect > < path d = "M2 12h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_CENTER : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;