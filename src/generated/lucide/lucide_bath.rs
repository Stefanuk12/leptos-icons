use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 6 6.5 3.5a1.5 1.5 0 0 0-1-.5C4.683 3 4 3.683 4 4.5V17a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5" ></ path > < line y1 = "5" y2 = "7" x1 = "10" x2 = "8" ></ line > < line x1 = "2" x2 = "22" y1 = "12" y2 = "12" ></ line > < line x1 = "7" x2 = "7" y2 = "21" y1 = "19" ></ line > < line x2 = "17" x1 = "17" y1 = "19" y2 = "21" ></ line > < / > } } pub const LUCIDE_BATH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;