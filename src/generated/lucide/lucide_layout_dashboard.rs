use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "9" width = "7" y = "3" x = "3" rx = "1" ></ rect > < rect rx = "1" height = "5" x = "14" y = "3" width = "7" ></ rect > < rect width = "7" y = "12" height = "9" rx = "1" x = "14" ></ rect > < rect width = "7" x = "3" y = "16" rx = "1" height = "5" ></ rect > < / > } } pub const LUCIDE_LAYOUT_DASHBOARD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;