use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "1" width = "7" height = "9" y = "3" ></ rect > < rect y = "3" height = "5" x = "14" rx = "1" width = "7" ></ rect > < rect width = "7" y = "12" x = "14" height = "9" rx = "1" ></ rect > < rect height = "5" width = "7" rx = "1" y = "16" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;