use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "19 20 9 12 19 4 19 20" ></ polygon > < line x1 = "5" y2 = "5" x2 = "5" y1 = "19" ></ line > < / > } } pub const LUCIDE_SKIP_BACK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;