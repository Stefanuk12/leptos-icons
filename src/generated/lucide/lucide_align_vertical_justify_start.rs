use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "14" height = "6" rx = "2" x = "5" y = "16" ></ rect > < rect rx = "2" x = "7" width = "10" y = "6" height = "6" ></ rect > < path d = "M2 2h20" ></ path > < / > } } pub const LUCIDE_ALIGN_VERTICAL_JUSTIFY_START : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round")] } ;