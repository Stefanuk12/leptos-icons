use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "7" height = "9" rx = "1" y = "3" x = "3" ></ rect > < rect height = "5" width = "7" rx = "1" y = "3" x = "14" ></ rect > < rect height = "9" y = "12" x = "14" rx = "1" width = "7" ></ rect > < rect y = "16" rx = "1" x = "3" width = "7" height = "5" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;