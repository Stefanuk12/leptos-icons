use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" y1 = "12" x2 = "10" x1 = "6" ></ line > < line x2 = "8" x1 = "8" y1 = "10" y2 = "14" ></ line > < line y1 = "13" x1 = "15" y2 = "13" x2 = "15.01" ></ line > < line y2 = "11" x2 = "18.01" y1 = "11" x1 = "18" ></ line > < rect width = "20" rx = "2" height = "12" y = "6" x = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;