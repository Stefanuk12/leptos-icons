use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.6 3.6A2 2 0 0 1 5 3h14a2 2 0 0 1 2 2v14a2 2 0 0 1-.59 1.41" ></ path > < path d = "M3 8.7V19a2 2 0 0 0 2 2h10.3" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M13 13a3 3 0 1 0 0-6H9v2" ></ path > < path d = "M9 17v-2.3" ></ path > < / > } } pub const LUCIDE_SQUARE_PARKING_OFF : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24")] } ;