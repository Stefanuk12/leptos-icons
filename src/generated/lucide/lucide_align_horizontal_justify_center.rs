use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" x = "2" width = "6" y = "5" rx = "2" ></ rect > < rect height = "10" y = "7" x = "16" width = "6" rx = "2" ></ rect > < path d = "M12 2v20" ></ path > < / > } } pub const LUCIDE_ALIGN_HORIZONTAL_JUSTIFY_CENTER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;