use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "16" width = "14" x = "5" height = "6" rx = "2" ></ rect > < rect rx = "2" y = "6" width = "10" x = "7" height = "6" ></ rect > < path d = "M2 2h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_START : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;