use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "9" width = "7" rx = "1" y = "3" x = "3" ></ rect > < rect width = "7" x = "14" y = "3" height = "5" rx = "1" ></ rect > < rect x = "14" rx = "1" y = "12" width = "7" height = "9" ></ rect > < rect height = "5" rx = "1" y = "16" width = "7" x = "3" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;