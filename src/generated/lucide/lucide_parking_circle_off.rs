use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m5 5 14 14" ></ path > < path d = "M13 13a3 3 0 1 0 0-6H9v2" ></ path > < path d = "M9 17v-2.34" ></ path > < / > } } pub const LucideParkingCircleOff : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none")] } ;