use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "6" x2 = "10" y2 = "12" ></ line > < line x2 = "8" y2 = "14" y1 = "10" x1 = "8" ></ line > < line x1 = "15" y1 = "13" y2 = "13" x2 = "15.01" ></ line > < line x1 = "18" x2 = "18.01" y1 = "11" y2 = "11" ></ line > < rect width = "20" height = "12" x = "2" y = "6" rx = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;