use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 6 6.5 3.5a1.5 1.5 0 0 0-1-.5C4.683 3 4 3.683 4 4.5V17a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5" ></ path > < line y2 = "7" x1 = "10" y1 = "5" x2 = "8" ></ line > < line y2 = "12" x1 = "2" y1 = "12" x2 = "22" ></ line > < line x1 = "7" x2 = "7" y2 = "21" y1 = "19" ></ line > < line y1 = "19" y2 = "21" x1 = "17" x2 = "17" ></ line > < / > } } pub const LUCIDE_BATH : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;