use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "16" width = "14" height = "6" x = "5" rx = "2" ></ rect > < rect rx = "2" x = "7" y = "2" width = "10" height = "6" ></ rect > < path d = "M2 12h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_CENTER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;