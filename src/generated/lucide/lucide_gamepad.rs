use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "10" y2 = "12" y1 = "12" x1 = "6" ></ line > < line y1 = "10" y2 = "14" x1 = "8" x2 = "8" ></ line > < line y2 = "13" x1 = "15" y1 = "13" x2 = "15.01" ></ line > < line x2 = "18.01" x1 = "18" y1 = "11" y2 = "11" ></ line > < rect width = "20" x = "2" y = "6" rx = "2" height = "12" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;