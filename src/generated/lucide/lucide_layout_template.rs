use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "7" width = "18" y = "3" rx = "1" ></ rect > < rect height = "7" y = "14" x = "3" width = "9" rx = "1" ></ rect > < rect rx = "1" x = "16" height = "7" width = "5" y = "14" ></ rect > < / > } } pub const LUCIDE_LAYOUT_TEMPLATE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;