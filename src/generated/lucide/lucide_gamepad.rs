use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "10" y2 = "12" x1 = "6" y1 = "12" ></ line > < line x2 = "8" y2 = "14" x1 = "8" y1 = "10" ></ line > < line x2 = "15.01" y1 = "13" x1 = "15" y2 = "13" ></ line > < line y1 = "11" y2 = "11" x1 = "18" x2 = "18.01" ></ line > < rect y = "6" rx = "2" height = "12" width = "20" x = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;