use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path clip - rule = "evenodd" fill - rule = "evenodd" d = "M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16ZM8.798 7.45c.512-.67 1.135-.95 1.702-.95s1.19.28 1.702.95a.75.75 0 0 0 1.192-.91C12.637 5.55 11.596 5 10.5 5s-2.137.55-2.894 1.54A5.205 5.205 0 0 0 6.83 8H5.75a.75.75 0 0 0 0 1.5h.77a6.333 6.333 0 0 0 0 1h-.77a.75.75 0 0 0 0 1.5h1.08c.183.528.442 1.023.776 1.46.757.99 1.798 1.54 2.894 1.54s2.137-.55 2.894-1.54a.75.75 0 0 0-1.192-.91c-.512.67-1.135.95-1.702.95s-1.19-.28-1.702-.95a3.505 3.505 0 0 1-.343-.55h1.795a.75.75 0 0 0 0-1.5H8.026a4.835 4.835 0 0 1 0-1h2.224a.75.75 0 0 0 0-1.5H8.455c.098-.195.212-.38.343-.55Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_CURRENCY_EURO : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("fill" , "currentColor") , ("viewBox" , "0 0 20 20")] } ;