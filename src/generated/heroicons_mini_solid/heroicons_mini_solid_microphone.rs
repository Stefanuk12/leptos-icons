use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 4a3 3 0 0 1 6 0v6a3 3 0 1 1-6 0V4Z" ></ path > < path d = "M5.5 9.643a.75.75 0 0 0-1.5 0V10c0 3.06 2.29 5.585 5.25 5.954V17.5h-1.5a.75.75 0 0 0 0 1.5h4.5a.75.75 0 0 0 0-1.5h-1.5v-1.546A6.001 6.001 0 0 0 16 10v-.357a.75.75 0 0 0-1.5 0V10a4.5 4.5 0 0 1-9 0v-.357Z" ></ path > < / > } } pub const HeroiconsMiniSolidMicrophone : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 20 20") , ("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor") , ("data-slot" , "icon")] } ;