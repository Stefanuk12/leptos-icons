use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "16" rx = "2" width = "14" x = "5" height = "6" ></ rect > < rect height = "6" x = "7" width = "10" y = "6" rx = "2" ></ rect > < path d = "M2 2h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_START : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round")] } ;